import express from "express";
import cors from "cors";
import authRoutes from "./routes/auth.route.js";
import userRoutes from "./routes/user.route.js";
import { ENV_PORT } from "./utils/envProvider.js";
import authTokenmiddleware from "./middlewares/auth.middleware.js";

const PORT = ENV_PORT;
const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors());

app.use("/api/auth", authRoutes);
app.use("/api/user", authTokenmiddleware, userRoutes);

app.listen(PORT, () => {
  console.log(`app link: http://localhost:${PORT}`);
});
