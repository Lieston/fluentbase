use crate::RuntimeContext;
use fluentbase_types::IJournaledTrie;
use rwasm::{core::Trap, Caller};

pub struct SyscallComputeRoot;

impl SyscallComputeRoot {
    pub fn fn_handler<DB: IJournaledTrie>(
        mut caller: Caller<'_, RuntimeContext<DB>>,
        output32_offset: u32,
    ) -> Result<(), Trap> {
        let root = Self::fn_impl(caller.data_mut());
        caller.write_memory(output32_offset, &root)?;
        Ok(())
    }

    pub fn fn_impl<DB: IJournaledTrie>(ctx: &mut RuntimeContext<DB>) -> [u8; 32] {
        let result = ctx.jzkt().compute_root();
        result
    }
}
