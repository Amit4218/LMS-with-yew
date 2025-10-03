import prisma from "../lib/prisma.js";

export const enroll = async (req, res) => {
  try {
    const courseId = req.params.courseId;
    const userId = req.user.userId;

    const course = await prisma.courses.findUnique({
      where: {
        courseId: courseId,
      },
      select: {
        courseId: true,
        courseName: true,
        description: true,
        language: true,
      },
    });

    if (!course) {
      return res.status(404).json({
        message: "Course Dosen't Exists",
      });
    }

    await prisma.userCourses.create({
      data: {
        courseId: course.courseId,
        userId: userId,
        courseName: course.courseName,
        description: course.description,
        language: course.language,
        updatedAt: new Date(),
      },
    });

    return res.status(200).json({
      message: "Enrolled Successfully",
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const deEnroll = async (req, res) => {
  try {
    const enrolledCourseId = req.params.enrolledCourseId;

    await prisma.userCourses.delete({
      where: {
        userCourseId: enrolledCourseId,
      },
    });

    res.status(200).json({
      message: "De-enrolled Successfully",
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const addCourse = async (req, res) => {
  try {
    const userId = req.user.userId;
    const { courseName, description, language, courseTitles, courseVideos } =
      req.body;

    await prisma.courses.create({
      data: {
        creatorId: userId,
        courseName: courseName,
        description: description,
        language: language,
        courseTitles,
        courseVideos,
        updatedAt: new Date(),
      },
    });

    res.status(201).json({
      message: "Course Created Successfully",
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const userCourses = async (req, res) => {
  try {
    const userId = req.user.userId;

    const UserCourses = await prisma.userCourses.findMany({
      where: {
        userId: userId,
      },
      select: {
        courseId: true,
        userCourseId: true,
        courseName: true,
        description: true,
        language: true,
      },
    });

    res.status(200).json({
      message: "Success",
      Courses: UserCourses,
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};

export const getCourse = async (req, res) => {
  try {
    const courseId = req.params.courseId;

    const course = await prisma.courses.findUnique({
      where: {
        courseId: courseId,
      },
      select: {
        courseId: true,
        courseName: true,
        description: true,
        language: true,
        courseTitles: true,
        courseVideos: true,
      },
    });

    return res.status(200).json({
      message: "Success",
      Course: course,
    });
  } catch (error) {
    return res.status(500).json({
      message: "Something Went Wrong",
      error: error.message,
    });
  }
};
