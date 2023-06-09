/**
 * Module dependencies.
 */
const { jwt } = require("../middleware");
const router = require("express").Router();
const authController = require("../controllers/auth.controller");

/* POST login listing. */
router.post("/login", authController.login);

/* POST register listing. */
router.post("/register", authController.register);

/* POST logout listing. */
router.post("/logout", jwt.verifyToken, authController.logout);

module.exports = router;
