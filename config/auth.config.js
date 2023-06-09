require("dotenv").config();

module.exports = {
  secret: process.env.JWT_TOKEN_SECRET || "secret",
  expiresIn: process.env.JWT_TOKEN_EXPIRES_IN || "1d",
};
