use std::io;

use identity::account::Account;
use identity::account::Command;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::IotaDID;

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

  let mut antibody_test = String::new();
  println!("Currently has COVID antibodies: ");
  match io::stdin().read_line(&mut antibody_test){
    Ok(_) => {
      println!("You entered: {}", antibody_test.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  let mut active_covid_infection = String::new();
  println!("SARS-CoV-2 positive (If true, specify date of positive test): ");
  match io::stdin().read_line(&mut active_covid_infection){
    Ok(_) => {
      println!("You entered: {}", active_covid_infection.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  // Create a new Account with Stronghold as the storage adapter
  let account: Account = Account::builder().build().await?;

  // Create a new Identity with default settings
  let snapshot1: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document: &IotaDID = snapshot1.identity().try_did()?;

  // Add a new Ed25519 Verification Method to the identity
  let command: Command = Command::create_method().fragment("key-1").finish()?;

  // Process the command and update the identity state.
  account.update_identity(document, command).await?;

  // Create a subject DID for the recipient of a `UniversityDegree` credential.
  let subject_key: KeyPair = KeyPair::new_ed25519()?;
  let subject_did: IotaDID = IotaDID::new(subject_key.public().as_ref())?;

  // Create the actual Verifiable Credential subject.
  // I thought this part would add an atribute to the json file given in the link provided when running. It did not :(
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject_did.as_str(),
    "test_results": {
      "vaccine_type_1": vaccine_type_1,     
      "vaccine_date_1": vaccine_date_1,
      "vaccine_provider_1": vaccine_provider_1,
      "vaccine_type_2": vaccine_type_2,     
      "vaccine_date_2": vaccine_date_2,
      "vaccine_provider_2": vaccine_provider_2,
      "antibody_test": antibody_test,
      "active_covid_infection": active_covid_infection,
    }
  }))?;

  // Issue an unsigned Credential...
  let mut credential: Credential = Credential::builder(Default::default())
    .issuer(Url::parse(document.as_str())?)
    .type_("TestResults")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account.sign(document, "key-1", &mut credential).await?;

  match io::stdin().read_line(&mut snapshot1.identity().this_message_id().to_string()){
    Ok(_) => {
      println!("Copy this link into a browser to view your secure vaccination certificate: https://explorer.iota.org/mainnet/message/{}", snapshot1.identity().this_message_id().to_string());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }

  Ok(())

}