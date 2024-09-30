#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User{
    username:String,
    id:u64,
    useremail:String,
    created_at:u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Project{
    id:u64,
    by:String,
    projectname:String,
    githublink:String,
    description:String,
    technologyused:String,
    created_at:u64,
    
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EnquireAboutProject{
    id:u64,
    by:String,
    enquire:String,
    projectname:String,
    usernamemail:String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CommentToProject{
    id:u64,
    by:String,
    project:String,
    comment:String,
    useremail:String,
    created_at:u64
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Project {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Project {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for EnquireAboutProject {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnquireAboutProject {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for CommentToProject {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CommentToProject {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// thread
thread_local! {
    static MEMORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>>=RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER:RefCell<IdCell>=RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0).expect("Cannot create a counter")
    );
    static PROJECTS_STORAGE:RefCell<StableBTreeMap<u64,Project,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static ENQUIRE_STORAGE:RefCell<StableBTreeMap<u64,EnquireAboutProject,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
    static COMMENTS_STORAGE:RefCell<StableBTreeMap<u64,CommentToProject,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static USERS_STORAGE:RefCell<StableBTreeMap<u64,User,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}


#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct UserPayload{
    username:String,
    useremail:String,   
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]

struct ProjectPayload{
    by:String,
    projectname:String,
    githublink:String,
    description:String,
    technologyused:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct EnquirePayload{
    by:String,
    enquire:String,
    projectname:String,
    usernamemail:String,

 }
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct CommentPayload{
    by:String,
    project:String,
    comment:String,
    useremail:String,
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct DeleteProjectPayload{
    projectid:u64,
    username:String
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct UpdateProjectPayload{
    projectid:u64,
    by:String,
    projectname:String,
    githublink:String,
    description:String,
    technologyused:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct SearchProjectPayload{
  projectid:u64,
}


//function to register user
#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, String> {
    // Validate the payload to ensure that the required fields are present
  
    if payload.username.is_empty()
        ||payload.useremail.is_empty()
    
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.useremail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    // Ensure email address uniqueness and username
    let email_exists:bool = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.useremail == payload.useremail)
    });
    if email_exists {
        return Err("Email already exists".to_string());
    }

   let username_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.username == payload.username)
});
if username_exists {
    return Err("The username already exists".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
   
    let newuser = User {
        username:payload.username,
        id,
        useremail:payload.useremail,
        created_at:time()
       
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, newuser.clone()));

    Ok(newuser)
}


//function where user post a project


#[ic_cdk::update]
fn post_a_project(payload:ProjectPayload)->Result<Project, String>{

      // Validate the payload to ensure that the required fields are present
      if payload.by.is_empty()
      || payload.projectname.is_empty()
      || payload.githublink.is_empty()
      ||payload.description.is_empty()
      ||payload.technologyused.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that user exists
   
       let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.username == payload.by)
    });
    
    if !checkuser_exists {
        return Err("You are not registered".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
    let new_project=Project{
        id,
        by:payload.by,
        projectname:payload.projectname,
        githublink:payload.githublink,
        description:payload.description,
        technologyused:payload.technologyused,
        created_at:time()
          };

        PROJECTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_project.clone()));

    Ok(new_project)
}

// //Function to retrieve all available projects

#[ic_cdk::query]
fn get_all_projects() -> Result<Vec<Project>, String> {

    let projects = PROJECTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<Project>>()
    });

    if  projects.is_empty() {
        return Err("Currently no projects available.".to_string());
    }

    else {
        Ok(projects)
    }

}


// users enquire about a project
#[ic_cdk::update]
fn users_enquire_about_a_project(payload:EnquirePayload)->Result<EnquireAboutProject,String>{
   

      // Validate the payload to ensure that the required fields are present
      if payload.by.is_empty()
      || payload.projectname.is_empty()
      ||payload.enquire.is_empty()
      ||payload.usernamemail.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.usernamemail.contains('@') {
        return Err("enter correct email format".to_string());
    }
//ensure that the project actually exists
let checkproject_exists:bool=PROJECTS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.projectname == payload.projectname)
});
if !checkproject_exists {
    return Err("project does not exist".to_string());
}
    let id = ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("Cannot increment ID counter");

    let new_enquire=EnquireAboutProject{
    id,
    by:payload.by,
    projectname:payload.projectname,
    enquire:payload.enquire,
    usernamemail:payload.usernamemail,
    created_at:time()
     };
ENQUIRE_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_enquire.clone()));

return Ok(new_enquire);
}

// //users update project 
#[ic_cdk::update]
fn update_project_details(payload:UpdateProjectPayload)->Result<Project,String>{
     if payload.by.is_empty()
        ||payload.projectname.is_empty()
        || payload.githublink.is_empty()
        || payload.description.is_empty()
        ||payload.technologyused.is_empty()
    {
        return Err("Ensure all credentials are provided".to_string());
    }
   
    
  
    
match PROJECTS_STORAGE.with(|service|service.borrow().get(&payload.projectid)){
    Some(mut projo)=>{    
                        projo.by=payload.by;
                        
                        projo.projectname=payload.projectname;
                        projo.githublink=payload.githublink;
                        projo.description=payload.description;
                        projo.technologyused=payload.technologyused;
                        do_insert(&projo);
                        Ok(projo)
                        
    }
    None=>Err("could not update project details".to_string()),
}

}

// users search for a project
#[ic_cdk::query]
fn search_for_a_project(payload:SearchProjectPayload)->Result<Project,String>{
    let project = PROJECTS_STORAGE.with(|storage| storage.borrow().get(&payload.projectid));
    match project {
        Some(project) => Ok(project),
        None => Err("project does not exist.".to_string()),
    }
    
}
//owner delete project 
#[ic_cdk::update]
  fn delete_project(payload:DeleteProjectPayload)->Result<String,String>{
 //verify its the owner of project is deleteing it
 let checkuser_exists:bool=PROJECTS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.by == payload.username)
});
if !checkuser_exists {
    return Err("ONLY OWNER".to_string());
}
    match PROJECTS_STORAGE.with(|storage|storage.borrow_mut().remove(&payload.projectid)){
        Some(_val)=>Ok("you have successfully deletede PROJECT".to_string()),
        None=>Err("coulde not delete the project".to_string(),)
    }
  }



//  users comments about project platform


  #[ic_cdk::update]
  fn users_commets_about_project(payload:CommentPayload)->Result<CommentToProject,String>{
    if payload.by.is_empty()
    ||payload.useremail.is_empty()
    || payload.comment.is_empty()
    || payload.project.is_empty()
     {
        return Err("some fields are missing".to_string());
     }
     //check if user is registered
    let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.username == payload.by)
    });
    if !checkuser_exists {
        return Err("You are not registered".to_string());
    }

    //check if project exists

    let checkproject_exists:bool=PROJECTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.projectname == payload.project)
    });
    if !checkproject_exists {
        return Err("project does not exists".to_string());
    }
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_comment=CommentToProject{
        id,
        by:payload.by,
        project:payload.project,
        comment:payload.comment,
        useremail:payload.useremail,
        created_at:time()
     };
     COMMENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_comment.clone()));
     return Ok(new_comment);
  }

//helper unction for updates
fn do_insert(project:&Project){
    PROJECTS_STORAGE.with(|service|service.borrow_mut().insert(project.id,project.clone()));
}
 ic_cdk::export_candid!();