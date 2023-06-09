"use strict";

const bcrypt = require("bcrypt");
const { faker } = require("@faker-js/faker");

module.exports = {
  up() {
    const categories = [...Array(3)].map((_) => ({
      name: faker.name.firstName(),
      username: faker.internet.userName(),
      password: bcrypt.hashSync("password", 10),
      createdAt: new Date(),
      updatedAt: new Date(),
    }));
    return categories;
  },

  down() {
    return null;
  },
};
