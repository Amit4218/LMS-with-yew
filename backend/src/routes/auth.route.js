import express from "express";
import { login, logout, register } from "../controllers/auth.controller.js";
import authTokenmiddleware from "../middlewares/auth.middleware.js";

const router = express.Router();

router.post("/login", login);
router.post("/register", register);
router.post("/logout", authTokenmiddleware, logout);

export default router;
