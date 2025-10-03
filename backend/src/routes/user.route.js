import express from "express";
import { addCourse, deEnroll, enroll } from "../controllers/user.controller";

const router = express.Router();

router.post("/enroll/:courseId", enroll);
router.post("/de-enroll/:courseId", deEnroll);
router.post("/add-course", addCourse);
// router.delete("/delete-course/:courseId");

export default router;
