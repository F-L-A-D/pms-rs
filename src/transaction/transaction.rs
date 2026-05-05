use crate::domain::inventory::HotelInventory;

pub fn run_in_transaction<T, F, R>(
    repo: &mut R,
    inventory: &mut HotelInventory,
    f: F,
) -> Result<T, String>
where
    F: FnOnce(&mut R, &mut HotelInventory) -> Result<T, String>,
    R: Clone,
{
    
    let repo_backup = repo.clone();
    let inventory_backup = inventory.clone();

    let result = f(repo, inventory);

    if result.is_err() {
        *repo = repo_backup;
        *inventory = inventory_backup;
    }

    result
}