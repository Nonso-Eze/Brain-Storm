import { Controller, Get, Param, Query, Patch, Delete, Body, UseGuards } from '@nestjs/common';
import { UsersService } from './users.service';
import { ApiBearerAuth, ApiTags } from '@nestjs/swagger';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';
import { RolesGuard } from '../auth/roles.guard';
import { Roles } from '../auth/roles.decorator';
import {
  Body,
  Controller,
  ForbiddenException,
  Get,
  Param,
  Patch,
  Request,
  UseGuards,
} from '@nestjs/common';
import { ApiBearerAuth, ApiTags } from '@nestjs/swagger';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';
import { UsersService } from './users.service';
import { UpdateUserDto } from './dto/update-user.dto';

@ApiTags('users')
@ApiBearerAuth()
@Controller('users')
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  @Get(':id')
  findOne(@Param('id') id: string) {
    return this.usersService.findById(id);
  }

  @UseGuards(JwtAuthGuard)
  @Patch(':id')
  async update(
    @Param('id') id: string,
    @Body() dto: UpdateUserDto,
    @Request() req: { user: { id: string } },
  ) {
    if (req.user.id !== id) {
      throw new ForbiddenException('You can only update your own profile');
    }
    return this.usersService.update(id, dto);
  }
}

@ApiTags('admin')
@ApiBearerAuth()
@Controller('admin/users')
@UseGuards(JwtAuthGuard, RolesGuard)
export class AdminUsersController {
  constructor(private readonly usersService: UsersService) {}

  @Get()
  @Roles('admin')
  findAll(
    @Query('page') page?: number,
    @Query('limit') limit?: number,
    @Query('role') role?: string,
    @Query('isVerified') isVerified?: string,
    @Query('search') search?: string,
  ) {
    return this.usersService.findAll({
      page: page ? Number(page) : 1,
      limit: limit ? Number(limit) : 10,
      role,
      isVerified: isVerified === 'true' ? true : isVerified === 'false' ? false : undefined,
      search,
    });
  }

  @Patch(':id/role')
  @Roles('admin')
  changeRole(@Param('id') id: string, @Body('role') role: string) {
    return this.usersService.changeRole(id, role);
  }

  @Patch(':id/ban')
  @Roles('admin')
  banUser(@Param('id') id: string, @Body('isBanned') isBanned: boolean) {
    return this.usersService.banUser(id, isBanned);
  }

  @Delete(':id')
  @Roles('admin')
  deleteUser(@Param('id') id: string) {
    return this.usersService.softDelete(id);
  }
}
