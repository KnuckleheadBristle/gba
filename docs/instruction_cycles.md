# Instruction cycle breakdown

There are a number of terms mentioned in this document, they are outlined below:

Transaction types:
| TRANS[1:0] | Transaction type | Description                                    |
| ---------- | ---------------- | ---------------------------------------------- |
| 00         | I cycle          | Internal (address-only) next cycle             |
| 01         | C cycle          | Coprocessor transfer next cycle                |
| 10         | N cycle          | Memory access to next address is nonsequential |
| 11         | S cycle          | Memory access to next address is sequential    |

Sizes:
| Letter    | Meaning   | Description                              |
| --------- | --------- | ---------------------------------------- |
| w         | word      | 32-bit data access or ARM opcode fetch   |
| h         | halfword  | 16-bit data access or THUMB opcode fetch |
| b         | byte      | 8-bit data access                        |

## Branch and ARM branch with link

There are three cycles to this instruction:

The first cycle:

- calculate destination while prefetching from current PC
  - Done in all cases as it is already too late by decision time

The second:

- Perform a fetch from the branch destination
  - Return address is stored in r14 if the link bit is set
The third (final):

- Perform a fetch from the destination +i (to refil the instruction pipeline)
  - If instruction is branch with link, 4 is subtracted from r14 to ensure return subroutines work correctly

| Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 |
| ----- | --------- | ----- | ----- | --------- | ------------- | ----- |
| 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
| 2     | pc'       | w'/h' | 0     | (pc')     | S cycle       | 0     |
| 3     | pc'+i     | w'/h' | 0     | (pc'+i)   | S cycle       | 0     |
|       | pc'+2i    | w'/h' |       |           |               |       |
|       |           |       |       |           |               |       |

## Thumb branch with link

This instruction comprises of two consecutive Thumb instructions, and takes four cycles.

The first instruction: Adds the offset to the value of PC, stores in LR (r14) (this takes one cycle)

The second instruction: Similar to the ARM BL instruction, takes three cycles

Cycle 1:

- Calculates the final branch destination while performing a prefetch from the current PC
Cycle 2:

- Performs a fetch from the branch destination
  - Stores return address in r14 (because link)

Cycle 3:

- Performs a fetch from destination +2, refills instruction pipeline
  - Subtract 2 from r14 to ensure subsequent subroutines work

| Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 |
| ----- | --------- | ----- | ----- | --------- | ------------- | ----- |
| 1     | pc + 4    | h     | 0     | (pc + 4)  | S cycle       | 0     |
| 2     | pc + 6    | h     | 0     | (pc + 6)  | N cycle       | 0     |
| 3     | pc'       | h     | 0     | (pc')     | S cycle       | 0     |
| 4     | pc' + 2   | h     | 0     | (pc' + 2) | S cycle       | 0     |
|       | pc' + 4   |       |       |           |               |       |

## Branch and exchange

The branch and exchange operation takes three cycles.

Cycle 1:

- Extracts branch destination and new core state from register source, while performing prefetch on current PC
  - prefetch is performed in all cases as it is already too late by decision time.

Cycle 2:

- Perform a fetch from the branch destination using the new instruction width
  - Dependant on state that has been selected

Cycle 3:

- Perform a fetch from the desination plus 2 or 4 (depending on the selected state)
  - Refilling instruction pipeline

| Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 | Tbit  |
| ----- | --------- | ----- | ----- | --------- | ------------- | ----- | ----- |
| 1     | pc + 2i   | w/h   | 0     | (pc + 2i) | N cycle       | 0     | t     |
| 2     | pc'       | w'/h' | 0     | (pc')     | S cycle       | 0     | t'    |
| 3     | pc'+i'    | w'/h' | 0     | (pc'+'i)  | S cycle       | 0     | t'    |
|       | pc' + 2i' |       |       |           |               |       |       |

## Data operations

A data operation instruction executes in a single data path cycle unless the shift is determined by the contents of a register.

Order of register reading:

1. Read the first register onto the A bus
2. Read second register or immediate field onto B bus

Instruction prefetches occur at the same time as a data operation, incrementing the PC.

If a register specifies the shift length, an additional data path cycle occurs before the operation to copy the bottom 8 bits of that register to the holding latch in the shifter. The instruction prefetch occurs during the first cycle.

As this operation is internal, the address remains stable through both cycles, the memory manager merging this internal cycle with a following sequential access.

As the PC can be one or more of the register operands, when the PC is the destination external bus activity may be affected. When the result is written to the PC, the instruction pipeline is invalidated and the processor takes the address for the next instruction prefetch from the ALU rather than the address incrementer, refilling the instruction pipeline before any further execution takes place. Exceptions are locked out during this time.

PSR is identical in timing characterisics as data operations except that PC is never used as a source or destination register.

| Type                  | Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 |
| ----                  | ----- | --------- | ----- | ----- | --------- | ------------- | ----- |
| Normal                | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | S cycle       | 0     |
|                       |       | pc+3i     |       |       |           |               |       |
| dest=pc               | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                       | 2     | pc'       | w/h   | 0     | (pc')     | S cycle       | 0     |
|                       | 3     | pc'+i     | w/h   | 0     | (pc'+i)   | S cycle       | 0     |
|                       |       | pc'+2i    |       |       |           |               |       |
| shift(Rs)             | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | I cycle       | 0     |
|                       | 2     | pc+3i     | w/h   | 0     | -         | S cycle       | 1     |
|                       |       | pc+3i     |       |       |           |               |       |
| shift(Rs), dest=pc    | 1     | pc+8      | w     | 0     | (pc+8)    | I cycle       | 0     |
|                       | 2     | pc+12     | w     | 0     | -         | N cycle       | 1     |
|                       | 3     | pc'       | w     | 0     | (pc')     | S cycle       | 0     |
|                       | 4     | pc'+4     | w     | 0     | (pc'+4)   | S cycle       | 0     |
|                       |       | pc'+8     |       |       |           |               |       |

## Multiply and Multiply Accumulate

I don't want to do this because it has a lot of tables, so I'll leave it for later.

## Load register

This instruction takes a variable number of cycles, but there are three defined steps.

Step 1:

- Calculate the address to be loaded

Step 2:

- Fetch the data from memory
  - Perform base register modification if required

Step 3:

- Transfer the data to the destination register (external memory is not used)
  - This is normally merged with the next prefetch

| Type      | Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 | Prot1 |
| --------- | ----- | --------- | ----- | ----- | --------- | ------------- | ----- | ----- |
| normal    | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     | s     |
|           | 2     | pc'       | w/h   | 0     | (pc')     | I cycle       | 1     | u/s   |
|           | 3     | pc+3i     | w/h/b | 0     | -         | S cycle       | 1     | s     |
|           |       | pc+3i     |       |       |           |               |       |       |
| dest=pc   | 1     | pc+8      | w     | 0     | (pc+8)    | N cycle       | 0     | s     |
|           | 2     | da        | w/h/b | 0     | pc'       | I cycle       | 1     | u/s   |
|           | 3     | pc+12     | w     | 0     | -         | N cycle       | 1     | s     |
|           | 4     | pc'       | w     | 0     | (pc')     | S cycle       | 0     | s     |
|           | 5     | pc'+4     | w     | 0     | (pc'+4)   | S cycle       | 0     | s     |
|           |       | pc'+8     |       |       |           |               |       |       |

The base or destination (or both) may be the PC. If the PC is affected by the instruction, the prefetch sequence changes. If the data fetch aborts, the processor prevents modification of the destination register.

## Store register

There are two cycles:

Cycle 1:

- Calculate the address to be stored

Cycle 2:

- Perform the base modification and write data to memory (if required)

| Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 | Prot1 |
| ----- | --------- | ----- | ----- | --------- | ------------- | ----- | ----- |
| 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     | s     |
| 2     | da        | b/h/w | 1     | Rd        | N cycle       | 1     | t     |
|       | pc+3i     |       |       |           |               |       |       |

t is either 0 when the T bit is specified in the instruction, or C at all other times.

## Load multiple registers

This instruction takes four cycles.

Cycle 1:

- Calculate the address of the first word to be transferred, while performing prefetch from memory

Cycle 2:

- Fetch the first word and perform base modification

Cycle 3:

- Move the first word to the appropriate destination register, fetch the second word from memory
  - Latches the modified base internally in case of an abort
  - This cycle is repeated for any subsequent fetches until the last data word has been accessed

Cycle 4:

- Move the last word to its destination register.
  - May be merged with the next instruction prefetch to form a single memory N-cycle

The instruction continues to completion when an abort occurs, but all register writing is prevented. The final cycle is instead changed to restore the modified base register from cycle 3.

If the PC is in the register list to be loaded, the processor invalidates the pipeline, because the PC is always the last to load, an abort at any point prevents the PC from being written to.

| Type                      | Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 |
| ------------------------- | ----- | --------- | ----- | ----- | --------- | ------------- | ----- |
| 1 register dest=pc        | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                           | 2     | da        | w     | 0     | pc'       | I cycle       | 1     |
|                           | 3     | pc+3i     | w/h   | 0     | -         | N cycle       | 1     |
|                           | 4     | pc'       | w/h   | 0     | (pc')     | S cycle       | 0     |
|                           | 5     | pc'+i     | w/h   | 0     | (pc'+i)   | S cycle       | 0     |
|                           |       | pc'+2i    |       |       |           |               |       |
| n registers (n>1)         | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                           | 2     | da        | w     | 0     | da        | S cycle       | 1     |
|                           | -     | da++      | w     | 0     | (da++)    | S cycle       | 1     |
|                           | n     | da++      | w     | 0     | (da++)    | S cycle       | 1     |
|                           | n+1   | da++      | w     | 0     | (da++)    | I cycle       | 1     |
|                           | n+2   | pc+3i     | w     | 0     | -         | S cycle       | 1     |
|                           |       | pc+3i     |       |       |           |               |       |
| n registers (n>1) incl pc | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                           | 2     | da        | w     | 0     | da        | S cycle       | 1     |
|                           | -     | da++      | w     | 0     | (da++)    | S cycle       | 1     |
|                           | n     | da++      | w     | 0     | (da++)    | S cycle       | 1     |
|                           | n+1   | da++      | w     | 0     | pc'       | I cycle       | 1     |
|                           | n+2   | pc+3i     | w/h   | 0     | -         | N cycle       | 1     |
|                           | n+3   | pc'       | w/h   | 0     | (pc')     | S cycle       | 0     |
|                           | n+4   | pc'+i     | w/h   | 0     | (pc'+1)   | S cycle       | 0     |
|                           |       | pc'+2i    |       |       |           |               |       |

## Store multiple registers

This instruction is very similar to the above, but without the last cycle, meaning there are only two cycles

Cycle 1:

- The address of the first word to be stored is calculated

Cycle 2:

- Base modification is performed
  - Data is written to memory

| Type                      | Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 |
| ------------------------- | ----- | --------- | ----- | ----- | --------- | ------------- | ----- |
| 1 register                | 1     | pc+2i     | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                           | 2     | da        | w     | 1     | R         | N cycle       | 1     |
|                           |       | pc+3i     |       |       |           |               |       |
| n registers (n>1)         | 1     | pc+8      | w/h   | 0     | (pc+2i)   | N cycle       | 0     |
|                           | 2     | da        | w     | 1     | R         | S cycle       | 1     |
|                           | -     | da++      | w     | 1     | R'        | S cycle       | 1     |
|                           | n     | da++      | w     | 1     | R''       | S cycle       | 1     |
|                           | n+1   | da++      | w     | 1     | R'''      | S cycle       | 1     |
|                           |       | pc+12     |       |       |           |               |       |

## Data swap

The data swap instruction is similar in operation to the load and store register instructions, and takes 4 cycles

Cycle 1:

- The address of the register value to be stored is calculated

Cycle 2:

- Data is fetched from external memory

Cycle 3:

- The contens of the source register are written to the external memory

Cycle 4:

- The data read during cycle 3 is written into the destination register

| Cycle | Address   | Size  | Write | Data      | TRANS[1:0]    | Prot0 | Lock  |
| ----- | --------- | ----- | ----- | --------- | ------------- | ----- | ----- |
| 1     | pc+8      | w     | 0     | (pc+8)    | N cycle       | 0     | 0     |
| 2     | Rn        | w/b   | 0     | (Rn)      | N cycle       | 1     | 1     |
| 3     | Rn        | w/b   | 1     | Rm        | I cycle       | 1     | 1     |
| 4     | pc+12     | w     | 0     | -         | S cycle       | 1     | 0     |
|       | pc+12     |       |       |           |               |       |       |
