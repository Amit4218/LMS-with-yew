/*
  Warnings:

  - You are about to drop the column `registeredUser` on the `Courses` table. All the data in the column will be lost.
  - Added the required column `description` to the `UserCourses` table without a default value. This is not possible if the table is not empty.
  - Added the required column `language` to the `UserCourses` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Courses" DROP COLUMN "registeredUser";

-- AlterTable
ALTER TABLE "UserCourses" ADD COLUMN     "description" TEXT NOT NULL,
ADD COLUMN     "language" TEXT NOT NULL;
