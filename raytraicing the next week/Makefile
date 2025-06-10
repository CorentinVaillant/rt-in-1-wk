CC = gcc
CXX = g++
CPPFLAGS = -Wextra -Wall -pedantic
CPP_DEBUG_FLAGS = -g
CPP_RELEASE_FLAGS = -O3 -DNDEBUG

ifeq ($(DEBUG),yes)
	CPPFLAGS += $(CPP_DEBUG_FLAGS)
else
	CPPFLAGS += $(CPP_RELEASE_FLAGS)
endif

BUILD_DIR = ./build
SOURCE_DIR = ./src

EXEC = out.out
SRC = $(wildcard $(SOURCE_DIR)/*.cpp)
OBJ = $(patsubst %.cpp,$(BUILD_DIR)/%.o,$(notdir $(SRC)))

all: $(BUILD_DIR) $(EXEC)
ifeq ($(DEBUG),yes)
	@echo "Generating in debug mod, CPPFLAGS : $(CPPFLAGS)" 
else
	@echo "Generating in release mod, CPPFLAGS : $(CPPFLAGS)" 
endif

$(BUILD_DIR):
	mkdir -p $@

$(EXEC): $(OBJ)
	$(CXX) -o $@ $^ $(CPPFLAGS)

$(BUILD_DIR)/%.o: $(SOURCE_DIR)/%.cpp
	$(CXX) -o $@ -c $< $(CPPFLAGS)

clean:
	rm -f $(BUILD_DIR)/*.o
	rm -f $(EXEC)

run : all
	./$(EXEC)