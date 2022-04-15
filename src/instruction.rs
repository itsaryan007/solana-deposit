// instruction.rs -> program API, (de)serializing instruction data

pub enum DespositInstruction {


    /// Starts the trade by creating and populating a deposit account and transferring ownership of the given temp token account to the PDA
   ///
   ///
   /// Accounts expected:
   ///
   /// 0. `[signer]` The account of the person initializing the deposit process
   /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
   /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
   /// 3. `[writable]` The deposit account, it will hold all necessary info about the trade.
   /// 4. `[]` The rent sysvar
   /// 5. `[]` The token program


   InitDeposit{
       //The amount party A suppose to deposit
       amount: u64
   },

   ReqWithdraw{
       //The amount party B suppose to withdraw
        amount:u64
   }
}

impl DepositInstruction {
    /// Unpacks a byte buffer into a [DepositInstruction](enum.DepositInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitDeposit {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::ReqWithdraw{
                amount: Self::unpack_amount(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}