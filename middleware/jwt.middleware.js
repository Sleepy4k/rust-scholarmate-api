/**
 * Module dependencies.
 */
const jwt = require("jsonwebtoken");
const config = require("../config/auth.config.js");

/**
 * Verify jwt token for validation.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.verifyToken = async (req, res, next) => {
  let token = req.session.token;

  if (!token) {
    return res.status(403).json({
      status: "error",
      message: "No token provided!",
      data: {},
    });
  }

  jwt.verify(token, config.secret, (err, decoded) => {
    if (err) {
      return res.status(401).json({
        status: "error",
        message: "Unauthorized!",
        data: {},
      });
    }
    req.userId = decoded.id;
    next();
  });
};
