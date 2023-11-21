#!/bin/bash

# FILEPATH: /c:/Users/jcvit/School/CDT/blueteamtool/sshpooky/setup.sh

# Check if a debian user is provided as the first argument
if [ $# -lt 1 ]; then
  echo "Error: Please provide a debian user as the first argument."
  exit 1
fi

# Assign the debian user to a variable
debian_user=$1

# Ensure that bash is the user's default shell
chsh -s /bin/bash $debian_user

# Copy the login_anim.sh script to the user's home directory
cp login_anim.sh /home/$debian_user/

# Append a call to the login_anim.sh script to the user's .bashrc file
echo "/home/$debian_user/login_anim.sh" >> /home/$debian_user/.bashrc

# Append a call to kill the login_anim.sh script to the user's .bashrc file
echo "kill -9 $$" >> /home/$debian_user/.bashrc
