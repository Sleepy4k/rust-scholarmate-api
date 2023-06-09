"use strict";

const factories = require("../factories/20230210183740-seed-users");

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.bulkInsert("users", factories.up(), {});
  },

  async down(queryInterface, Sequelize) {
    await queryInterface.bulkDelete("users", factories.down(), {});
  },
};
