import sys

def main():
    print("Hello, World!")

    # Check if there are any command-line arguments passed
    if len(sys.argv) > 1:
        # Print the first parameter after the script name
        print(f"First parameter: {sys.argv[1]}")
    else:
        print("No parameters were passed.")

if __name__ == "__main__":
    main()
