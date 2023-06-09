require("dotenv").config();

module.exports = {
  development: {
    username: process.env.DEV_DB_USERNAME || "root",
    password: process.env.DEV_DB_PASSWORD || "",
    database: process.env.DEV_DB_NAME || "schoolar",
    host: process.env.DEV_DB_HOSTNAME || "localhost",
    port: process.env.DEV_DB_PORT || 3306,
    dialect: "mysql",
    dialectOptions: {
      bigNumberStrings: true,
    },
  },
  test: {
    username: process.env.CI_DB_USERNAME || "root",
    password: process.env.CI_DB_PASSWORD || "",
    database: process.env.CI_DB_NAME || "schoolar",
    host: process.env.CI_DB_HOSTNAME || "localhost",
    port: process.env.CI_DB_PORT || 3306,
    dialect: "mysql",
    dialectOptions: {
      bigNumberStrings: true,
    },
  },
  production: {
    username: process.env.PROD_DB_USERNAME || "root",
    password: process.env.PROD_DB_PASSWORD || "",
    database: process.env.PROD_DB_NAME || "schoolar",
    host: process.env.PROD_DB_HOSTNAME || "localhost",
    port: process.env.PROD_DB_PORT || 3306,
    dialect: "mysql",
    dialectOptions: {
      bigNumberStrings: true,
    },
  },
};
