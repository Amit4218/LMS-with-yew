/*
  Warnings:

  - Added the required column `description` to the `Courses` table without a default value. This is not possible if the table is not empty.
  - Added the required column `language` to the `Courses` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Courses" ADD COLUMN     "description" TEXT NOT NULL,
ADD COLUMN     "language" TEXT NOT NULL;
