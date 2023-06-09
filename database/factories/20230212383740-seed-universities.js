"use strict";

const { faker } = require("@faker-js/faker");

module.exports = {
  up() {
    const categories = [...Array(5)].map((_) => ({
      name: faker.name.jobTitle(),
      description: faker.lorem.paragraph(),
      major: faker.name.jobArea(),
      quantity: faker.datatype.number(),
      createdAt: new Date(),
      updatedAt: new Date(),
    }));

    return categories;
  },

  down() {
    return null;
  },
};
