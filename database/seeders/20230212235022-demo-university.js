"use strict";

const factories = require("../factories/20230212383740-seed-universities");

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.bulkInsert("universities", factories.up(), {});
  },

  async down(queryInterface, Sequelize) {
    await queryInterface.bulkDelete("universities", factories.down(), {});
  },
};
