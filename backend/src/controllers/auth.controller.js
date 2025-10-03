import jwt from "jsonwebtoken";
import bcrypt from "bcrypt";
import { JWT_SECRET } from "../utils/envProvider.js";
import prisma from "../lib/prisma.js";

export const login = async (req, res) => {
  try {
    const { email, password } = req.body;

    if (!email || !password) {
      return res.status(404).json({
        message: "All feilds required",
      });
    }

    const existingUser = prisma.users.findUnique({
      where: {
        email: email,
      },
    });

    if (!existingUser) {
      return res.status(404).json({
        message: "Invalid Credientials",
      });
    }

    const passwordMatch = await bcrypt.compare(password, existingUser.password);

    if (!passwordMatch) {
      return res.status(404).json({
        message: "Invalid Credientials",
      });
    }

    const session = await prisma.session.create({
      data: {
        userId: existingUser.userId,
        updatedAt: new Date(),
      },
    });

    const jwtToken = jwt.sign(
      {
        userId: existingUser.userId,
        email: existingUser.email,
        sessionId: session.sessionId,
      },
      { JWT_SECRET },
      { expiresIn: "7d" }
    );

    return res.status(200).json({
      message: "Login Successfull",
      token: jwtToken,
      User: { userId: existingUser.userId, email: existingUser.email },
    });
  } catch (error) {
    res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const register = async (req, res) => {
  try {
    const { email, password } = req.body;

    if (!email || !password) {
      return res.status(404).json({
        message: "All feilds required",
      });
    }

    const existingUser = prisma.users.findUnique({
      where: {
        email: email,
      },
    });

    if (existingUser) {
      return res.status(409).json({
        message: "User Already Exists",
      });
    }

    const newUser = await prisma.users.create({
      data: {
        email: email,
        password: password,
        updatedAt: new Date(),
      },
    });

    const session = await prisma.session.create({
      data: {
        userId: newUser.userId,
        updatedAt: new Date(),
      },
    });

    const jwtToken = jwt.sign(
      {
        userId: existingUser.userId,
        email: existingUser.email,
        sessionId: session.sessionId,
      },
      { JWT_SECRET },
      { expiresIn: "7d" }
    );

    return res.status(200).json({
      message: "Login Successfull",
      token: jwtToken,
      User: { userId: newUser.userId, email: newUser.email },
    });
  } catch (error) {
    res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const logout = async (req, res) => {
  try {
    const sessionId = req.user.sessionId;

    await prisma.session.update({
      where: {
        sessionId: sessionId,
      },
      data: {
        updatedAt: new Date(),
      },
    });

    return res.status(200).json({
      message: "Logout Successfully",
    });
  } catch (error) {
    res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};
