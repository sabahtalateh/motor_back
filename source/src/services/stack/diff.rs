use crate::services::stack::Block;
use crate::handlers::stack::UpdateBlock;

pub fn get_new_blocks<'a, 'b>(
    existent_blocks: &'b Vec<Block>,
    updated_blocks: &'b Vec<UpdateBlock>,
) -> Vec<&'b UpdateBlock> {
    updated_blocks
        .iter()
        .filter(|u| {
            if u.id.is_none() {
                return true;
            }

            let updated_block_id_presented_in_existent_blocks_ids =
                existent_blocks.iter().any(|e| match &u.id {
                    None => false,
                    Some(id) => id == &e.id,
                });

            return !updated_block_id_presented_in_existent_blocks_ids;
        })
        .collect()
}

pub fn get_deleted_blocks<'a, 'b>(
    existent_blocks: &'b Vec<Block>,
    updated_blocks: &'b Vec<UpdateBlock>,
) -> Vec<&'b Block> {
    existent_blocks.iter()
        .filter(|e| {
            let existent_block_presented_in_updated = updated_blocks.iter().any(|u| {
               match &u.id {
                   None => false,
                   Some(id) => id == &e.id
               }
            });

            !existent_block_presented_in_updated
        })
        .collect()
}
