import { Controller, Get, Post, Param, Body, UseGuards } from '@nestjs/common';
import { ApiTags } from '@nestjs/swagger';
import { StellarService } from './stellar.service';
import { JwtAuthGuard } from '../auth/jwt-auth.guard';
import { RolesGuard } from '../auth/roles.guard';
import { Roles } from '../auth/roles.decorator';

@ApiTags('stellar')
@Controller('stellar')
export class StellarController {
  constructor(private stellarService: StellarService) {}

  @Get('balance/:publicKey')
  getBalance(@Param('publicKey') publicKey: string) {
    return this.stellarService.getAccountBalance(publicKey);
  }

  @Post('mint')
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles('admin')
  mintCredential(@Body() body: { recipientPublicKey: string; courseId: string }) {
    return this.stellarService.issueCredential(body.recipientPublicKey, body.courseId);
  }
}

@ApiTags('credentials')
@Controller('credentials')
@UseGuards(JwtAuthGuard, RolesGuard)
export class CredentialsController {
  constructor(private stellarService: StellarService) {}

  @Post('issue')
  @Roles('admin')
  issueCredential(@Body() body: { recipientPublicKey: string; courseId: string }) {
    return this.stellarService.issueCredential(body.recipientPublicKey, body.courseId);
  }
}
