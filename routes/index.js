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
        title: name,
        author: "Benjamin4k",
      });
});

/* GET home api page. */
router.get("/api", function (req, res, next) {
    res.status(200).json({
        status: "success",
        message: "Welcome to schoolar mates api",
        data: {}
    })
});

/* RESOURCE auth. */
router.use("/api/auth", require("./auth"));

/* RESOURCE university. */
router.use("/api/university", require("./university"));

module.exports = router;
