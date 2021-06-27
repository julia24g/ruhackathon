use std::io;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Ask for inputs
  println!("First dose (enter 'N/A' at inputs if not recieved): ");

  let mut vaccine_type_1 = String::new();
  println!("Insert vaccine type: ");
  match io::stdin().read_line(&mut vaccine_type_1){
    Ok(_) => {
      println!("You entered: {}", vaccine_type_1.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  let mut vaccine_date_1 = String::new();
  println!("Insert date of vaccination (YYYY/MM/DD): ");
  match io::stdin().read_line(&mut vaccine_date_1){
    Ok(_) => {
      println!("You entered: {}", vaccine_date_1.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  let mut vaccine_provider_1 = String::new();
  println!("Insert vaccine provider: ");
  match io::stdin().read_line(&mut vaccine_provider_1){
    Ok(_) => {
      println!("You entered: {}", vaccine_provider_1.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  println!("");

  println!("Second dose (enter 'N/A' at inputs if not recieved): ");

  let mut vaccine_type_2 = String::new();
  println!("Insert vaccine type: ");
  match io::stdin().read_line(&mut vaccine_type_2){
    Ok(_) => {
      println!("You entered: {}", vaccine_type_2.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  let mut vaccine_date_2 = String::new();
  println!("Insert date of vaccination (YYYY/MM/DD): ");
  match io::stdin().read_line(&mut vaccine_date_2){
    Ok(_) => {
      println!("You entered: {}", vaccine_date_2.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  let mut vaccine_provider_2 = String::new();
  println!("Insert vaccine provider: ");
  match io::stdin().read_line(&mut vaccine_provider_2){
    Ok(_) => {
      println!("You entered: {}", vaccine_provider_2.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  // Create a new Account with Stronghold as the storage adapter
  let account: Account = Account::builder().build().await?;

  // Create a new Identity with default settings
  let snapshot1: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document1: &IotaDID = snapshot1.identity().try_did()?;

  //println!("[Example] Local Snapshot = {:#?}", snapshot1);
  //println!("[Example] Local Document = {:#?}", snapshot1.identity().to_document()?);
  //println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  match io::stdin().read_line(&mut snapshot1.identity().this_message_id().to_string()){
    Ok(_) => {
      println!("Copy this link into a browser to view your secure vaccination certificate: https://explorer.iota.org/mainnet/message/{}", snapshot1.identity().this_message_id().to_string());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document1).await?;

  //println!("[Example] Tangle Document = {:#?}", resolved);

  Ok(())


}