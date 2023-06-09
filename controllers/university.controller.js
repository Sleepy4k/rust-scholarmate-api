/**
 * Module dependencies.
 */
const models = require("../models");

/**
 * Display a listing of the resource.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.index = async (req, res, next) => {
  try {
    await models.university
      .findAll({
        attributes: {
          exclude: ["createdAt", "updatedAt"],
        },
      })
      .then((universities) => {
        res.status(202).json({
          status: "success",
          message: "Universities fetched successfully",
          data: universities || {},
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
 * Store a newly created resource in storage.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.store = async (req, res, next) => {
  const { name, description, major, quantity } = req.body;

  if (!name || !description || !major || !quantity) {
    return res.status(400).json({
      status: "error",
      message: "Bad request",
      data: {},
    });
  }

  try {
    await models.university
      .create({
        name,
        description,
        major,
        quantity,
      })
      .then((university) => {
        res.status(201).json({
          status: "success",
          message: "University created successfully",
          data: university || {},
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
 * Display the specified resource.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.show = async (req, res, next) => {
  const id = req.params.id;

  try {
    await models.university
      .findByPk(id, {
        rejectOnEmpty: true,
        attributes: {
          exclude: ["createdAt", "updatedAt"],
        },
      })
      .then((university) => {
        res.status(206).json({
          status: "success",
          message: "University fetched successfully",
          data: university || {},
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
 * Update the specified resource in storage.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.update = async (req, res, next) => {
  const id = req.params.id;
  const { name, description, major, quantity } = req.body;

  if (!name || !description || !major || !quantity) {
    return res.status(400).json({
      status: "error",
      message: "Bad request",
      data: {},
    });
  }

  try {
    await models.university
      .findByPk(id, {
        rejectOnEmpty: true,
        attributes: {
          exclude: ["createdAt", "updatedAt"],
        },
      })
      .then((university) => {
        university.name = name;
        university.description = description;
        university.major = major;
        university.quantity = quantity;

        return university.save();
      })
      .then((university) => {
        res.status(202).json({
          status: "success",
          message: "University updated successfully",
          data: university || {},
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
 * Remove the specified resource from storage.
 *
 * @param  Request  req
 * @param  Response  res
 * @param  Next  next
 *
 * @return Array
 */
exports.destroy = async (req, res, next) => {
  const id = req.params.id;

  try {
    await models.university
      .findByPk(id, { rejectOnEmpty: true })
      .then((university) => {
        return university.destroy();
      })
      .then(() => {
        res.status(202).json({
          status: "success",
          message: "University deleted successfully",
          data: null,
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
