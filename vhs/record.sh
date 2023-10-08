
docker build -t vhs-tisq .

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CURRENT_FOLDER=$(pwd)
elif [[ "$OSTYPE" == "darwin"* ]]; then
    CURRENT_FOLDER=$(pwd)
elif [[ "$OSTYPE" == "cygwin" ]]; then
    CURRENT_FOLDER=$(pwd -W)
elif [[ "$OSTYPE" == "msys" ]]; then
    CURRENT_FOLDER=$(pwd -W)
elif [[ "$OSTYPE" == "win32" ]]; then
    CURRENT_FOLDER=$(pwd -W)
fi

docker run --rm -ti --privileged -v ${CURRENT_FOLDER}:/vhs vhs-tisq ./base.tape
