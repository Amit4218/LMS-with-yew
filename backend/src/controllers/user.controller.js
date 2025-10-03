import prisma from "../lib/prisma.js";

export const enroll = async (req, res) => {
  try {
    const courseId = req.params.courseId;
    const userId = req.user.userId;

    const course = await prisma.courses.findUnique({
      where: {
        courseId: courseId,
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
    const courseId = req.params.courseId;
    const userId = req.user.userId;

    await prisma.userCourses.delete({
      where: {
        userId: userId,
        courseId: courseId,
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
        courseTitles: [courseTitles],
        courseVideos: [courseVideos],
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
