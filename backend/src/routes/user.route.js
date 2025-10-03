import express from "express";
import {
  addCourse,
  deEnroll,
  enroll,
  getCourse,
  userCourses,
} from "../controllers/user.controller.js";

const router = express.Router();

router.post("/enroll/:courseId", enroll);
router.post("/de-enroll/:enrolledCourseId", deEnroll);
router.post("/add-course", addCourse);
router.get("/user-courses", userCourses);
router.get("/course/:courseId", getCourse);

// router.delete("/delete-course/:courseId");

export default router;
