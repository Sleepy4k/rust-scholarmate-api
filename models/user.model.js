"use strict";

const { Model } = require("sequelize");

module.exports = (sequelize, DataTypes) => {
  class User extends Model {
    /**
     * Helper method for defining associations.
     * This method is not a part of Sequelize lifecycle.
     * The `models/index` file will call this method automatically.
     */
    static associate(models) {
      // define association here
    }
  }
  User.init(
    {
      name: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Name must not be empty" } },
      },
      username: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Username must not be empty" } },
      },
      password: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Password must not be empty" } },
      },
    },
    {
      sequelize,
      modelName: "user",
      tableName: "users",
    }
  );
  return User;
};
