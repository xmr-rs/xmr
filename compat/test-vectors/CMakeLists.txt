cmake_minimum_required(VERSION 2.8.7)
project(gen_test_vectors C CXX)

find_package(Boost 1.58 QUIET REQUIRED COMPONENTS system filesystem thread date_time chrono regex serialization program_options)

set(CMAKE_BINARY_DIR "bin")

add_definitions("-DAUTO_INITIALIZE_EASYLOGGINGPP")

include_directories("../monero/contrib/epee/include")
include_directories("../monero/external/easylogging++")
include_directories("../monero/src/")
include_directories(${Boost_INCLUDE_DIRS})

add_subdirectory("../monero/contrib/epee/" ${CMAKE_CUURENT_BINARY_DIR}"/epee")
add_subdirectory("../monero/external/easylogging++/" ${CMAKE_CUURENT_BINARY_DIR}"/easylogging++/")
add_subdirectory("src/")
