/**
 * Module dependencies.
 */
const router = require("express").Router();
const name = require("../config/app.config").name;

/* GET home page. */
router.get("/", function (req, res, next) {
    res.status(200);
      res.render("pages/index", {
        node: "18.15.0",
        title: "Schoolar Mates",
        author: "Benjamin4k",
      });
});

/* GET home api page. */
router.get("/api", function (req, res, next) {
    res.status(200).json({
        status: "success",
        message: "Welcome to adjex api",
        data: {}
    })
});

/* RESOURCE quiz. */
router.use("/api/auth", require("./auth"));

module.exports = router;
