import os

os.system("cargo build --release")
os.system("del C:\\bin\\medialog.exe")
os.system("del C:\\bin\\ml.exe")
os.system("copy .\\target\\release\\medialog.exe C:\\bin\\medialog.exe")
os.system("copy .\\target\\release\\medialog.exe C:\\bin\\ml.exe")