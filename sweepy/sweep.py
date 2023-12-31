import os
import hashlib
import json

def hash_file(filepath):
    hasher = hashlib.md5()
    try:
        with open(filepath, 'rb') as f:
            buf = f.read()
            hasher.update(buf)
            f.close()
    except PermissionError:
        pass # TODO notify of permission issues
    return hasher.hexdigest()

def hash_directory(directory, blacklist=[]):
    
    print("Hashing directory: " + directory)
    items = {}
    for item in os.listdir(directory):
        path = os.path.join(directory, item)
        if path in blacklist:
            continue
        if os.path.isfile(path):
            items[item] = hash_file(path)
        elif os.path.isdir(path):
            items[item] = hash_directory(path)
    return items

def main(directories, blacklist=[]):
    all_hashes = {}
    for directory in directories:
        all_hashes[directory] = hash_directory(directory, blacklist)
    
    with open('directory_hashes.json', 'w') as f:
        json.dump(all_hashes, f, indent=4)

if __name__ == "__main__":
    # directories = ['/bin', '/boot', '/sys', '/home', '/root', '/tmp', '/usr', '/var', '/sbin', '/etc']
    directories = ['/bin', '/boot', '/home', '/root', '/tmp', '/usr', '/var', '/sbin', '/etc', '/sys/kernel', '/sys/module']
    blacklist = ['/sys/kernel/debug', '/sys/module/hid']
    main(directories, blacklist)