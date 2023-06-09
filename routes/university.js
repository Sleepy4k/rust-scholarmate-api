/**
 * Module dependencies.
 */
const { jwt } = require("../middleware");
const router = require("express").Router();
const universityController = require("../controllers/university.controller");

/* GET quiz listing. */
router.get("/", universityController.index);

/* POST quiz listing. */
router.post("/", jwt.verifyToken, universityController.store);

/* GET spesific quiz listing. */
router.get("/:id", universityController.show);

/* PUT spesific quiz listing. */
router.put("/:id", jwt.verifyToken, universityController.update);

/* DELETE spesific quiz listing. */
router.delete("/:id", jwt.verifyToken, universityController.destroy);

module.exports = router;
