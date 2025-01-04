import os
import subprocess


def append_to_file(data_iterable, file_path):
    is_first_iteration = True  # Flag to track the first iteration

    for data in data_iterable:
        mode = "w" if is_first_iteration else "a"  # Choose mode based on flag
        try:
            with open(file_path, mode, encoding="utf-8") as file:
                file.write(f"{data}\n")  # Write data followed by a newline
            print(f"Written to {file_path} in mode '{mode}'. Data: {data}")
        except Exception as e:
            print(f"Error writing to {file_path}: {e}")
        finally:
            is_first_iteration = False  # Update flag after first write
    return


waveform_dir = os.getcwd() + "/waveforms/"
design_names = os.listdir(waveform_dir)

result_path = os.path.join(waveform_dir, "resutl.txt")

contents = []

for design in design_names:
    design_dir = os.path.join(waveform_dir, design)
    for benchmark in os.listdir(design_dir):
        command = [
            "cargo",
            "run",
            "--example",
            "activity_factor",
            "--",
            design,
            benchmark,
        ]
        result = subprocess.run(
            command,
            stdout=subprocess.PIPE,  # Capture standard output
            stderr=subprocess.PIPE,  # Capture standard error
            text=True,  # Return output as strings instead of bytes
            check=True,  # Raise CalledProcessError for non-zero exit codes
        )
        contents.append(result.stdout)

append_to_file(contents, result_path)
