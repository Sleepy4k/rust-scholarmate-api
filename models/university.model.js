"use strict";

const { Model } = require("sequelize");

module.exports = (sequelize, DataTypes) => {
  class University extends Model {
    /**
     * Helper method for defining associations.
     * This method is not a part of Sequelize lifecycle.
     * The `models/index` file will call this method automatically.
     */
    static associate(models) {
      // define association here
    }
  }
  University.init(
    {
      name: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Name must not be empty" } },
      },
      description: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Description must not be empty" } },
      },
      major: {
        type: DataTypes.STRING,
        validate: { notEmpty: { msg: "Major must not be empty" } },
      },
      quantity: {
        type: DataTypes.INTEGER,
        validate: { notEmpty: { msg: "Quantity must not be empty" } },
      },
    },
    {
      sequelize,
      modelName: "university",
      tableName: "universities",
    }
  );
  return University;
};
