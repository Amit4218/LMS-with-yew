import jwt from "jsonwebtoken";
import bcrypt from "bcrypt";
import { JWT_SECRET } from "../utils/envProvider.js";
import prisma from "../lib/prisma.js";

export const login = async (req, res) => {
  try {
    const { email, password } = req.body;

    if (!email || !password) {
      return res.status(400).json({
        message: "All feilds required",
      });
    }

    const existingUser = await prisma.users.findUnique({
      where: {
        email: email,
      },
      select: {
        userId: true,
        email: true,
        password: true,
      },
    });

    if (!existingUser) {
      return res.status(400).json({
        message: "Invalid Credientials",
      });
    }

    const passwordMatch = await bcrypt.compare(password, existingUser.password);

    if (!passwordMatch) {
      return res.status(400).json({
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
      JWT_SECRET,
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
      return res.status(400).json({
        message: "All feilds required",
      });
    }

    const existingUser = await prisma.users.findUnique({
      where: { email },
    });

    if (existingUser) {
      return res.status(409).json({ message: "User already exists" });
    }

    const hashPassword = await bcrypt.hash(password, 10);

    const newUser = await prisma.users.create({
      data: {
        email: email,
        password: hashPassword,
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
        userId: newUser.userId,
        email: newUser.email,
        sessionId: session.sessionId,
      },
      JWT_SECRET,
      { expiresIn: "7d" }
    );

    return res.status(201).json({
      message: "Registeration Successfull",
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

export const getAllCourses = async (req, res) => {
  try {
    const courses = await prisma.courses.findMany();

    return res.status(200).json({
      message: "Success",
      courses: courses,
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something went wrong",
      error: error.message,
    });
  }
};
