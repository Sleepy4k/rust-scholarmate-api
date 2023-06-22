use validator::Validate;
use pretty_assertions::assert_eq;

#[test]
fn test_validate_email() {
  #[derive(Validate)]
  struct Test {
    #[validate(length(min = 1, max = 255), email)]
    email: String
  }

  let tests = vec![
    Test {
      email: "johndoe123@gmail.com".to_string()
    },
    Test {
      email: "johndoe123@yahoo.com".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_ok(), true);
  }
}

#[test]
fn test_validate_email_error() {
  #[derive(Validate)]
  struct Test {
    #[validate(length(min = 1, max = 255), email)]
    email: String
  }

  let tests = vec![
    Test {
      email: "johndoe123gmail.com".to_string()
    },
    Test {
      email: "johndoe123yahoo.com".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_err(), true);
  }
}

#[test]
fn test_validate_length() {
  #[derive(Validate)]
  struct Test {
    #[validate(length(min = 1, max = 255))]
    name: String
  }

  let tests = vec![
    Test {
      name: "John Doe".to_string()
    },
    Test {
      name: "Jane Doe".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_ok(), true);
  }
}

#[test]
fn test_validate_length_error() {
  #[derive(Validate)]
  struct Test {
    #[validate(length(min = 1, max = 255))]
    name: String
  }

  let tests = vec![
    Test {
      name: "".to_string()
    },
    Test {
      name: "".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_err(), true);
  }
}

#[test]
fn test_validate_positif_number() {
  #[derive(Validate)]
  struct Test {
    #[validate(custom = "validate_positif_number")]
    quantity: i32
  }

  fn validate_positif_number(number: i32) -> Result<(), validator::ValidationError> {
    if number < 0 {
      return Err(validator::ValidationError::new("Number must be positif"));
    }

    Ok(())
  }

  let tests = vec![
    Test {
      quantity: 1
    },
    Test {
      quantity: 2
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_ok(), true);
  }
}

#[test]
fn test_validate_positif_number_error() {
  #[derive(Validate)]
  struct Test {
    #[validate(custom = "validate_positif_number")]
    quantity: i32
  }

  fn validate_positif_number(number: i32) -> Result<(), validator::ValidationError> {
    if number < 0 {
      return Err(validator::ValidationError::new("Number must be positif"));
    }

    Ok(())
  }

  let tests = vec![
    Test {
      quantity: -1
    },
    Test {
      quantity: -2
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_err(), true);
  }
}

#[test]
fn test_validate_url() {
  #[derive(Validate)]
  struct Test {
    #[validate(url)]
    url: String
  }

  let tests = vec![
    Test {
      url: "https://google.com".to_string()
    },
    Test {
      url: "https://facebook.com".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_ok(), true);
  }
}

#[test]
fn test_validate_url_error() {
  #[derive(Validate)]
  struct Test {
    #[validate(url)]
    url: String
  }

  let tests = vec![
    Test {
      url: "google.com".to_string()
    },
    Test {
      url: "facebook.com".to_string()
    }
  ];

  for test in tests {
    let result = test.validate();

    assert_eq!(result.is_err(), true);
  }
}
