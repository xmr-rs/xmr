add_executable(gen_test_vectors main.cpp)
target_link_libraries(gen_test_vectors ${Boost_SYSTEM_LIBRARY} epee easylogging)
