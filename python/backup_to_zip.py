import zipfile, os
from pathlib import Path

def backup_to_zip(folder):
    folder = Path(folder)

    number = 1

    while True:
        zip_filename = Path(folder.parts[-1] + '_' + str(number) + '.zip')
        if not zip_filename.exists():
            break
        number = number + 1
        
    print(f'Creating {zip_filename}...')
    backup_zip = zipfile.ZipFile(zip_filename, 'w')

    for folder_name, subfolder, filenames in os.walk(folder):
        folder_names = Path(folder_name)
        print(f'Adding files in folder {folder_name}...')

        for filename in filenames:
            print(f'Adding files {filename}...')
            backup_zip.write(folder_name / filename)

    backup_zip.close
    print('Done')

backup_to_zip(Path.home() / 'spam')
