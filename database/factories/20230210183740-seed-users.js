"use strict";

const bcrypt = require("bcrypt");
const { faker } = require("@faker-js/faker");

module.exports = {
  up() {
    const categories = [...Array(3)].map((_) => ({
      email: faker.internet.email(),
      role: "user",
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
