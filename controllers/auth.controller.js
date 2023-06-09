/**
 * Module dependencies.
 */
const bcrypt = require("bcrypt");
const models = require("../models");
const jwt = require("jsonwebtoken");
const config = require("../config/auth.config.js");

/**
 * Handle user for login.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.login = async (req, res, next) => {
  const { email, password } = req.body;

  if (!email || !password) {
    return res.status(400).json({
      status: "error",
      message: "Bad request",
      data: {},
    });
  }

  try {
    await models.user
      .findOne({
        where: { email: email },
        attributes: {
          exclude: ["createdAt", "updatedAt"],
        },
      })
      .then((user) => {
        if (!user) {
          return res.status(200).json({
            status: "success",
            message: "Login failed",
            data: user || {},
          });
        }

        const passwordIsValid = bcrypt.compareSync(password, user.password);

        if (!passwordIsValid) {
          return res.status(401).send({
            status: "error",
            message: "Invalid Password!",
            data: {},
          });
        }

        const token = jwt.sign({ id: user.id }, config.secret, {
          expiresIn: 86400, // 24 hours
        });

        req.session.token = token;

        res.status(200).json({
          status: "success",
          message: "Login successful",
          data: user || {},
          token: token,
        });
      })
      .catch((error) => {
        res.status(500).json({
          status: "error",
          message: "Internal server error",
          data: error.message || {},
        });
      });
  } catch (error) {
    res.status(500).json({
      status: "error",
      message: "Internal server error",
      data: error.message || {},
    });
  }
};

/**
 * Handle user for register.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.register = async (req, res, next) => {
  const { email, password } = req.body;

  if (!email || !password) {
    return res.status(400).json({
      status: "error",
      message: "Bad request",
      data: {},
    });
  }

  try {
    await models.user
      .findOne({
        where: {
          email: email,
        },
      })
      .then((user) => {
        if (user) {
          res.status(200).json({
            status: "success",
            message: "User already exist",
            data: {},
          });
          return;
        } else {
          models.user
            .create({
              email: email,
              role: "user",
              password: bcrypt.hashSync(password, 10),
            })
            .then((user) => {
              res.status(200).json({
                status: "success",
                message: "Register successful",
                data: user || {},
              });
            })
            .catch((error) => {
              res.status(500).json({
                status: "error",
                message: "Internal server error",
                data: error.message || {},
              });
            });
        }
      })
      .catch((error) => {
        res.status(500).json({
          status: "error",
          message: "Internal server error",
          data: error.message || {},
        });
      });
  } catch (error) {
    res.status(500).json({
      status: "error",
      message: "Internal server error",
      data: error.message || {},
    });
  }
};

/**
 * Handle user for logout.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.logout = async (req, res, next) => {
  try {
    req.session = null;

    res.status(200).json({
      status: "success",
      message: "Logout successful",
      data: {},
      session: req.session,
    });
  } catch (error) {
    res.status(500).json({
      status: "error",
      message: "Internal server error",
      data: error.message || {},
    });
  }
};
