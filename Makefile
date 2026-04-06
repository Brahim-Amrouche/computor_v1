NAME = computor

SOURCES = $(wildcard src/*.rs)

all: $(NAME)

$(NAME): $(SOURCES)
	@echo "Building $(NAME)..."
	@cargo build --release
	@cp target/release/computor_v1 ./$(NAME)
	@echo "Executable $(NAME) created."

clean:
	@echo "Cleaning cargo cache..."
	@cargo clean

fclean: clean
	@echo "Removing $(NAME) executable..."
	@rm -f $(NAME)

re: fclean all

test:
	@echo "Running integration tests..."
	@cargo test

.PHONY: all clean fclean re test