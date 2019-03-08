#include <iostream>
#include <fstream>
#include <list>

int main(int argc, char *argv[]) {
  if (argc!=2) {
    std::cout << "Invalid argument(s).";
    return 1;
  }

  std::ifstream inputstream(argv[1]);
  if(!inputstream) {
    std::cout << "Cannot open file.";
    return -1;
  }

  std::list<char> inputarray;
  std::string line;
  //int inputlength=0;
  while(!inputstream.eof()) {
    inputstream.getline(line);
    inputarray.push_back(line);
  }

  inputstream.close();

  for(int i=0; i<=inputarray.size(); i++) {
    std::cout << inputarray.pop_front();
  }

  return 0;
}
