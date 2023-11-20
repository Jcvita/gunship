import os
import hashlib
import json

def hash_file(filepath):
    hasher = hashlib.md5()
    with open(filepath, 'rb') as f:
        buf = f.read()
        hasher.update(buf)
    return hasher.hexdigest()

def compare_hashes(system_hashes, stored_hashes):
    for item, system_hash in system_hashes.items():
        if item not in stored_hashes:
            print(f"A file has been added: {item}")
        elif isinstance(system_hash, dict):
            if isinstance(stored_hashes[item], dict):
                compare_hashes(system_hash, stored_hashes[item])
            else:
                print(f"Mismatch found for directory: {item}")
        else:
            if system_hash != stored_hashes[item]:
                print(f"Hash mismatch for file: {item}")
    for item in stored_hashes:
        if item not in system_hashes:
            print(f"A file has been removed: {item}")

def get_system_hashes(directory):
    items = {}
    for item in os.listdir(directory):
        path = os.path.join(directory, item)
        if os.path.isfile(path):
            items[item] = hash_file(path)
        elif os.path.isdir(path):
            items[item] = get_system_hashes(path)
    return items

def main(json_file, directory=None):
    with open(json_file, 'r') as f:
        stored_hashes = json.load(f)
    
    if directory:
        system_hashes = {directory: get_system_hashes(directory)}
        if directory in stored_hashes:
            compare_hashes(system_hashes[directory], stored_hashes[directory])
        else:
            print(f"No stored hashes for directory: {directory}")
    else:
        for dir_path, dir_hashes in stored_hashes.items():
            if os.path.exists(dir_path):
                system_hashes = get_system_hashes(dir_path)
                compare_hashes(system_hashes, dir_hashes)
            else:
                print(f"Directory does not exist: {dir_path}")

if __name__ == "__main__":
    json_file = 'directory_hashes.json'
    directory = input("Enter a directory to compare or press Enter to compare all: ").strip()
    if directory:
        main(json_file, directory)
    else:
        main(json_file)