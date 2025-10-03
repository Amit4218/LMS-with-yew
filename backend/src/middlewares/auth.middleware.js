import jwt from "jsonwebtoken";

const authTokenmiddleware = async (req, res, next) => {
  try {
    const token = req.headers.authorization;

    if (!token) {
      return res.status(401).json({ message: "Unauthorized" });
    }

    const decodedUser = jwt.verify(token, process.env.JWT_SECRET);

    req.user = decodedUser;

    next();
  } catch (error) {
    if (error.message == "jwt expired") {
      return res
        .status(401)
        .json({ message: "Unauthorized", error: error.name });
    }
    return res
      .status(500)
      .json({ message: "Unauthorized", error: error.message });
  }
};

export default authTokenmiddleware;
