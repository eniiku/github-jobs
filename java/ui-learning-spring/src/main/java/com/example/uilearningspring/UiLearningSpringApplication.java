package com.example.uilearningspring;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;


@SpringBootApplication
public class UiLearningSpringApplication {

	@Controller
	public class AboutController {
		@GetMapping("/about")
		public String about() {
			return "about";
		}
	}


	public static void main(String[] args) {
		SpringApplication.run(UiLearningSpringApplication.class, args);
	}
}

