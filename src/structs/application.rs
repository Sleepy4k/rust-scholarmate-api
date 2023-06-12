use serde::{Serialize};

use crate::structs::{ student::StudentStruct, university::UniversityStruct };

#[doc = "Application struct"]
#[derive(Debug, Serialize)]
pub struct ApplicationStruct {
  pub id: i32,
  pub student_id: i32,
  pub univ_id: i32,
  pub status: String,
}

#[doc = "Detail application struct"]
#[derive(Debug, Serialize)]
pub struct DetailApplicationStruct {
  pub id: i32,
  pub student: StudentStruct,
  pub university: UniversityStruct,
  pub status: String,
}