require("dotenv").config();

const fs = require("fs");
const path = require("path");
const Sequelize = require("sequelize");
const env = require("../config/app.config").env;
const basename = path.basename(__filename);
const { development, test, production } = require("../config/database.config");

const connection =
  env === "development" ? development : env === "test" ? test : production;

const sequelize = new Sequelize(
  connection.database,
  connection.username,
  connection.password,
  {
    host: connection.host,
    port: connection.port,
    dialect: connection.dialect,
    pool: {
      max: 5,
      min: 0,
      acquire: 30000,
      idle: 10000,
    },
  }
);

const db = {};

fs.readdirSync(__dirname)
  .filter((file) => {
    return (
      file.indexOf(".") !== 0 && file !== basename && file.slice(-3) === ".js"
    );
  })
  .forEach((file) => {
    const model = require(path.join(__dirname, file))(sequelize, Sequelize);
    db[model.name] = model;
  });

Object.keys(db).forEach((modelName) => {
  if (db[modelName].associate) {
    db[modelName].associate(db);
  }
});

db.Sequelize = Sequelize;
db.sequelize = sequelize;

module.exports = db;
