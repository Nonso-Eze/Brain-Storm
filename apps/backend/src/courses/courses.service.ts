import { Injectable, NotFoundException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Course } from './course.entity';

@Injectable()
export class CoursesService {
  constructor(@InjectRepository(Course) private repo: Repository<Course>) {}

  findAll() {
    return this.repo.find({ where: { isPublished: true } });
  }

  findOne(id: string) {
    return this.repo.findOne({ where: { id } });
  }

  create(data: Partial<Course>) {
    return this.repo.save(this.repo.create(data));
  }

  async update(id: string, data: Partial<Course>) {
    const course = await this.findOne(id);
    if (!course) throw new NotFoundException('Course not found');
    return this.repo.save({ ...course, ...data });
  }

  async delete(id: string) {
    const course = await this.findOne(id);
    if (!course) throw new NotFoundException('Course not found');
    return this.repo.remove(course);
  }
}
