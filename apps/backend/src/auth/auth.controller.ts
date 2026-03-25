import { Body, Controller, Get, Post, Req, UseGuards } from '@nestjs/common';
import { ApiTags } from '@nestjs/swagger';
import { AuthService } from './auth.service';
import { GoogleAuthGuard } from './google-auth.guard';
import { GoogleProfile } from './google.strategy';
import { IsEmail, IsString, MinLength } from 'class-validator';

class AuthDto {
  @IsEmail() email: string;
  @IsString() @MinLength(8) password: string;
}

@ApiTags('auth')
@Controller('auth')
export class AuthController {
  constructor(private authService: AuthService) {}

  @Post('register')
  register(@Body() dto: AuthDto) {
    return this.authService.register(dto.email, dto.password);
  }

  @Post('login')
  login(@Body() dto: AuthDto) {
    return this.authService.login(dto.email, dto.password);
  }

  // Redirects user to Google consent screen
  @UseGuards(GoogleAuthGuard)
  @Get('google')
  googleLogin() {
    // Guard handles the redirect — no body needed
  }

  // Google redirects back here after consent
  @UseGuards(GoogleAuthGuard)
  @Get('google/callback')
  googleCallback(@Req() req: { user: GoogleProfile }) {
    return this.authService.loginWithGoogle(req.user);
  }
}
